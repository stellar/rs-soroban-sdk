use std::fs;
use walkdir::WalkDir;
use syn::{File, Item, ItemFn, Stmt, Expr};
use quote::ToTokens;

// ===================== Finding =====================
#[derive(Debug)]
struct Finding {
    rule: String,
    severity: String,
    message: String,
    hint: String,
}

// ===================== Parser =====================
fn parse_file(path: &str) -> File {
    let code = fs::read_to_string(path).expect("❌ Failed to read file");
    syn::parse_file(&code).expect("❌ Failed to parse Rust file")
}

fn extract_functions(ast: &File) -> Vec<ItemFn> {
    ast.items.iter().filter_map(|i| {
        if let Item::Fn(f) = i { Some(f.clone()) } else { None }
    }).collect()
}

// ===================== Rules =====================
fn check_infinite_loop(func: &ItemFn) -> Option<Finding> {
    for stmt in &func.block.stmts {
        if let Stmt::Expr(Expr::Loop(_), _) = stmt {
            return Some(Finding {
                rule: "InfiniteLoop".into(),
                severity: "HIGH".into(),
                message: format!("Potential infinite loop in function `{}`", func.sig.ident),
                hint: ai_hint("InfiniteLoop"),
            });
        }
    }
    None
}

fn check_storage_abuse(func: &ItemFn) -> Option<Finding> {
    let src = func.to_token_stream().to_string();
    let count = src.matches("env.storage").count();
    if count > 5 {
        return Some(Finding {
            rule: "StorageAbuse".into(),
            severity: "MEDIUM".into(),
            message: format!("Heavy storage usage ({}x) in `{}`", count, func.sig.ident),
            hint: ai_hint("StorageAbuse"),
        });
    }
    None
}

// ===================== AI Hint =====================
fn ai_hint(rule: &str) -> String {
    match rule {
        "InfiniteLoop" =>
            "Add a break condition or use a bounded iterator / counter.".into(),
        "StorageAbuse" =>
            "Cache reads locally and batch writes to reduce ledger access.".into(),
        _ => "No AI hint available.".into(),
    }
}

// ===================== Engine =====================
fn analyze_path(path: &str) -> Vec<Finding> {
    let mut findings = vec![];

    for entry in WalkDir::new(path) {
        let e = entry.unwrap();
        if e.path().extension().map(|s| s == "rs").unwrap_or(false) {
            let ast = parse_file(e.path().to_str().unwrap());
            let funcs = extract_functions(&ast);

            for f in funcs {
                if let Some(x) = check_infinite_loop(&f) {
                    findings.push(x);
                }
                if let Some(x) = check_storage_abuse(&f) {
                    findings.push(x);
                }
            }
        }
    }

    findings
}

// ===================== Report =====================
fn print_report(findings: &[Finding]) {
    println!("\n📊 Soroban AI Static Analyzer Report");
    println!("==================================");

    if findings.is_empty() {
        println!("✅ No issues found. Contract looks clean.");
        return;
    }

    for (i, f) in findings.iter().enumerate() {
        println!("\n{}. ⚠ [{}] {}", i + 1, f.severity, f.rule);
        println!("   → {}", f.message);
        println!("   💡 AI Hint: {}", f.hint);
    }
}

// ===================== Main =====================
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).map(|s| s.as_str()).unwrap_or(".");

    println!("🔍 Running Soroban AI Static Analyzer on: {}", path);

    let findings = analyze_path(path);
    print_report(&findings);
}
