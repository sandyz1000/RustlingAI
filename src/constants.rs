#![allow(unused)]
use std::collections::HashMap;

pub const CODE_LANGUAGE_SUBSET: &[&str] = &[
    "python",
    "javascript",
    "java",
    "go",
    "bash",
    "c",
    "cpp",
    "csharp",
    "css",
    "diff",
    "graphql",
    "json",
    "kotlin",
    "less",
    "lua",
    "makefile",
    "markdown",
    "objectivec",
    "perl",
    "php",
    "php-template",
    "plaintext",
    "python-repl",
    "r",
    "ruby",
    "rust",
    "scss",
    "shell",
    "sql",
    "swift",
    "typescript",
    "vbnet",
    "wasm",
    "xml",
    "yaml",
];

pub const FOLDER_COLOR_OPTIONS: &[&str] = &[
    "#be123c", // rose-700
    "#6d28d9", // violet-700
    "#0369a1", // sky-700
    "#047857", // emerald-700
    "#b45309", // amber-700
];

pub const DEFAULT_SYSTEM_MESSAGE: &str =
    "You are ChatGPT, a large language model trained by OpenAI.
Carefully heed the user's instructions. 
Respond using Markdown.";

// languages that have translation files in `public/locales`
pub const I18N_LANGUAGES: &[&str] = &[
    // "ar",
    "da", "de", "en", "en-GB", "en-US", "es", "fr", "fr-FR", "it", "ja", "ms", "nb", "ro", "ru",
    "sv", // "ug",
    "yue", "zh", "zh-CN", "zh-HK", "zh-TW",
];

// languages that are selectable on the web page
pub const SELECTABLE_LANGUAGES: &[&str] = &[
    // "ar",
    "da", "de", // "en",
    "en-GB", "en-US", "es", // "fr",
    "fr-FR", "it", "ja", "ms", "nb", "ro", "ru", "sv", // "ug",
    "yue", // "zh",
    "zh-CN", // "zh-HK",
    "zh-TW",
];

// Define a struct for cost details (prompt and completion)
#[derive(Debug)]
pub struct CostDetail {
    pub price: f64,
    pub unit: u32,
}

// Define a struct for model cost (prompt and completion costs)
#[derive(Debug)]
pub struct ModelCost {
    pub prompt: CostDetail,
    pub completion: CostDetail,
}

lazy_static::lazy_static! {
    pub static ref MODEL_MAX_TOKEN: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("gpt-3.5-turbo", 4096);
        m.insert("gpt-3.5-turbo-0301", 4096);
        m.insert("gpt-3.5-turbo-0613", 4096);
        m.insert("gpt-3.5-turbo-16k", 16384);
        m.insert("gpt-3.5-turbo-16k-0613", 16384);
        m.insert("gpt-3.5-turbo-1106", 16384);
        m.insert("gpt-3.5-turbo-0125", 16384);
        m.insert("gpt-4", 8192);
        m.insert("gpt-4-0314", 8192);
        m.insert("gpt-4-0613", 8192);
        m.insert("gpt-4-32k", 32768);
        m.insert("gpt-4-32k-0314", 32768);
        m.insert("gpt-4-32k-0613", 32768);
        m.insert("gpt-4-1106-preview", 128000);
        m.insert("gpt-4-0125-preview", 128000);
        m.insert("gpt-4-turbo", 128000);
        m.insert("gpt-4-turbo-2024-04-09", 128000);
        m.insert("gpt-4o", 128000);
        m.insert("gpt-4o-2024-05-13", 128000);
        m
    };

    pub static ref LANGUAGE_CODE_TO_NAME: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        // m.insert("ar", "العربية");
        m.insert("da", "Dansk");
        m.insert("de", "Deutsch");
        m.insert("en", "English");
        m.insert("en-GB", "English (UK)");
        m.insert("en-US", "English (US)");
        m.insert("es", "Español");
        m.insert("fr", "Français");
        m.insert("fr-FR", "Français"); // Français (France). no need to include "France" at this time, as there is currently only one variant
        m.insert("it", "Italiano");
        m.insert("ja", "日本語");
        m.insert("ms", "Bahasa Melayu");
        m.insert("nb", "Norsk bokmål");
        m.insert("ro", "Română");
        m.insert("ru", "Русский");
        m.insert("sv", "Svenska");
        // m.insert("ug", "ئۇيغۇرچە");
        m.insert("yue", "廣東話");
        m.insert("zh", "中文");
        m.insert("zh-CN", "中文（简体）");
        m.insert("zh-HK", "廣東話"); // 中文（香港）. currently there is no support for `zh-HK`, so `zh-HK` will be regarded as `yue`
        m.insert("zh-TW", "中文（台灣）");
        m
    };

    pub static ref MODEL_COST: HashMap<&'static str, ModelCost> = {
        let model_cost: HashMap<&str, ModelCost> = HashMap::from([
            (
                "gpt-3.5-turbo",
                ModelCost {
                    prompt: CostDetail { price: 0.0015, unit: 1000 },
                    completion: CostDetail { price: 0.002, unit: 1000 },
                },
            ),
            (
                "gpt-3.5-turbo-0301",
                ModelCost {
                    prompt: CostDetail { price: 0.0015, unit: 1000 },
                    completion: CostDetail { price: 0.002, unit: 1000 },
                },
            ),
            (
                "gpt-3.5-turbo-0613",
                ModelCost {
                    prompt: CostDetail { price: 0.0015, unit: 1000 },
                    completion: CostDetail { price: 0.002, unit: 1000 },
                },
            ),
            (
                "gpt-3.5-turbo-16k",
                ModelCost {
                    prompt: CostDetail { price: 0.003, unit: 1000 },
                    completion: CostDetail { price: 0.004, unit: 1000 },
                },
            ),
            (
                "gpt-3.5-turbo-16k-0613",
                ModelCost {
                    prompt: CostDetail { price: 0.003, unit: 1000 },
                    completion: CostDetail { price: 0.004, unit: 1000 },
                },
            ),
            (
                "gpt-3.5-turbo-1106",
                ModelCost {
                    prompt: CostDetail { price: 0.001, unit: 1000 },
                    completion: CostDetail { price: 0.0015, unit: 1000 },
                },
            ),
            (
                "gpt-3.5-turbo-0125",
                ModelCost {
                    prompt: CostDetail { price: 0.0005, unit: 1000 },
                    completion: CostDetail { price: 0.0015, unit: 1000 },
                },
            ),
            (
                "gpt-4",
                ModelCost {
                    prompt: CostDetail { price: 0.03, unit: 1000 },
                    completion: CostDetail { price: 0.06, unit: 1000 },
                },
            ),
            (
                "gpt-4o",
                ModelCost {
                    prompt: CostDetail { price: 0.005, unit: 1000 },
                    completion: CostDetail { price: 0.015, unit: 1000 },
                },
            ),
            (
                "gpt-4o-2024-05-13",
                ModelCost {
                    prompt: CostDetail { price: 0.005, unit: 1000 },
                    completion: CostDetail { price: 0.015, unit: 1000 },
                },
            ),
        ]);
        model_cost
    };
}

pub const AVAILABLE_ENDPOINTS: &'static [&'static str] = &["https://api.openai.com/v1/chat/completions"];

pub const MODEL_OPTIONS: &[&str] = &[
    "gpt-3.5-turbo",
  "gpt-3.5-turbo-16k",
  "gpt-3.5-turbo-1106",
  "gpt-3.5-turbo-0125",
  "gpt-4",
  "gpt-4-32k",
  "gpt-4-1106-preview",
  "gpt-4-0125-preview",
  "gpt-4-turbo",
  "gpt-4-turbo-2024-04-09",
  "gpt-4o",
  "gpt-4o-2024-05-13",
  // "gpt-3.5-turbo-0301",
  // "gpt-4-0314",
  // "gpt-4-32k-0314",
];
