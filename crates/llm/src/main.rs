use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyList};

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let llm_handler = py.import_bound("llm_handler").expect("導入套件時發生錯誤");
        let prompt = format!(
            r#"
請簡潔回答以下問題，無需額外解釋：
問題：{}
        "#,
            "台灣的國慶日是幾月幾號？"
        );

        let model_kwargs = [
            (
                "model_path",
                "/home/sei/Program/llm-models/taide-8b-a.3-q4_k_m.gguf".into_py(py),
            ),
            ("n_gpu_layers", 12.into_py(py)),
            ("verbose", false.into_py(py)),
        ]
        .into_py_dict_bound(py);

        let llm = llm_handler
            .call_method("quantize_model", (), Some(&model_kwargs))
            .expect("讀取 llm 的過程發生錯誤");

        let call_kwargs = [(
            "stop",
            PyList::new_bound(py, vec!["\n", "問題:", "<|eot_id|>"]),
        )]
        .into_py_dict_bound(py);

        let result = llm_handler
            .call_method("inference", (llm, prompt), Some(&call_kwargs))
            .expect("推論過程發生錯誤");

        println!("{}", result.to_string());
    });
    Ok(())
}
