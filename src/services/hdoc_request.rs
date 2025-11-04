use std::collections::HashMap;

use reqwest::Client;

use crate::structs::{request_data::RequestData, request_result::RequestResult};

pub fn parse_heredoc_request(input: &str) -> Result<RequestData, String> {
    let mut method: Option<String> = None;
    let mut url: Option<String> = None;
    let mut headers = HashMap::new();
    let mut body: Option<String> = None;

    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();
        if let Some(tag) = line.strip_prefix("<<") {
            let tag = tag.trim();
            let mut content = String::new();
            i += 1;

            while i < lines.len() && lines[i].trim() != tag {
                content.push_str(lines[i]);
                content.push('\n');
                i += 1;
            }

            let content = content.trim().to_string();

            match tag.to_uppercase().as_str() {
                "METHOD" => method = Some(content),
                "URL" => url = Some(content),
                "HEADERS" => {
                    for line in content.lines() {
                        if let Some((k, v)) = line.split_once(':') {
                            headers.insert(k.trim().to_string(), v.trim().to_string());
                        }
                    }
                }
                "BODY" => body = Some(content),
                _ => {} // ignora blocos desconhecidos
            }
        }
        i += 1;
    }

    // 🔍 Validação obrigatória
    let method = method.ok_or("Bloco obrigatório <<METHOD ... METHOD>> ausente")?;
    let url = url.ok_or("Bloco obrigatório <<URL ... URL>> ausente")?;

    // validação de URL simples
    if !(url.starts_with("http://") || url.starts_with("https://")) {
        return Err(format!(
            "URL inválida: '{url}' (deve começar com http:// ou https://)"
        ));
    }

    // validação do método HTTP simples
    let valid_methods = ["GET", "POST", "PUT", "DELETE", "PATCH"];
    if !valid_methods.contains(&method.to_uppercase().as_str()) {
        return Err(format!(
            "Método HTTP inválido: '{}' (válidos: GET, POST, PUT, DELETE, PATCH)",
            method
        ));
    }

    Ok(RequestData {
        method: Some(method),
        url: Some(url),
        headers,
        body,
    })
}

pub async fn send_request(req: &RequestData) -> Result<RequestResult, String> {
    // configura cliente HTTP
    let client_builder = Client::builder();

    let client = client_builder
        .build()
        .map_err(|e| format!("client build error: {:?}", e))?;

    // define método e URL (normalizado em maiúsculas)
    let method = req.method.as_deref().unwrap_or("GET").to_uppercase();
    let url = req.url.as_deref().ok_or("Missing URL")?;

    let mut request = match method.to_uppercase().as_str() {
        "POST" => client.post(url),
        "PUT" => client.put(url),
        "DELETE" => client.delete(url),
        "PATCH" => client.patch(url),
        _ => client.get(url),
    };

    // headers
    for (k, v) in &req.headers {
        request = request.header(k, v);
    }

    // corpo (se houver). Em ambientes WASM (fetch), GET/HEAD não podem ter body.
    if let Some(body) = &req.body {
        let allow_body = matches!(method.as_str(), "POST" | "PUT" | "PATCH" | "DELETE");
        if allow_body {
            request = request.body(body.clone());
        }
    }

    // envia e processa resposta
    let response = request.send().await.map_err(|e| e.to_string())?;

    let status_code = response.status().as_u16();
    let mut headers_map = HashMap::new();

    for (k, v) in response.headers().iter() {
        headers_map.insert(k.to_string(), v.to_str().unwrap_or("").to_string());
    }

    let body = response.text().await.map_err(|e| e.to_string())?;

    Ok(RequestResult {
        status_code,
        headers: headers_map,
        body,
    })
}
