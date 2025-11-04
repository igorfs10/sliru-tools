use std::{collections::HashMap, time::Duration};

use reqwest::{Client, redirect::Policy};

use crate::structs::{request_data::RequestData, request_result::RequestResult};

pub fn parse_heredoc_request(input: &str) -> Result<RequestData, String> {
    let mut method: Option<String> = None;
    let mut url: Option<String> = None;
    let mut headers = HashMap::new();
    let mut cookies = HashMap::new();
    let mut body: Option<String> = None;
    let mut timeout_secs: Option<u64> = None;
    let mut follow_redirects: Option<bool> = None;

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
                "COOKIES" => {
                    for line in content.lines() {
                        if let Some((k, v)) = line.split_once(':') {
                            cookies.insert(k.trim().to_string(), v.trim().to_string());
                        }
                    }
                }
                "BODY" => body = Some(content),
                "TIMEOUT" => match content.trim().parse::<u64>() {
                    Ok(v) => timeout_secs = Some(v),
                    Err(_) => return Err(format!("Valor inválido em TIMEOUT: '{}'", content)),
                },
                "FOLLOW_REDIRECTS" => {
                    let val = content.trim().to_lowercase();
                    follow_redirects = Some(matches!(val.as_str(), "true" | "1" | "yes"));
                }
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
        cookies,
        body,
        timeout_secs,
        follow_redirects,
    })
}

pub async fn send_request(req: &RequestData) -> Result<RequestResult, String> {
    // configura cliente HTTP
    let mut client_builder = Client::builder();

    // timeout opcional
    if let Some(secs) = req.timeout_secs {
        client_builder = client_builder.timeout(Duration::from_secs(secs));
    } else {
        client_builder = client_builder.timeout(Duration::from_secs(30));
    }

    // redirecionamentos
    if let Some(follow) = req.follow_redirects {
        client_builder = client_builder.redirect(if follow {
            Policy::limited(10)
        } else {
            Policy::none()
        });
    }

    let client = client_builder.build().map_err(|e| e.to_string())?;

    // define método e URL
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

    // cookies
    if !req.cookies.is_empty() {
        let cookie_header = req
            .cookies
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("; ");
        request = request.header("Cookie", cookie_header);
    }

    // corpo (se houver)
    if let Some(body) = &req.body {
        request = request.body(body.clone());
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
