use pyo3::prelude::*;

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let llm_handler = py.import_bound("llm_handler").unwrap();

        let llm = llm_handler
            .call_method1(
                "load_gguf_model",
                ("/home/sei/Program/llm-models/taide-8b-a.3-q4_k_m.gguf",),
            )
            .expect("讀取 llm 的過程發生錯誤");

        let chain = llm_handler
            .call_method1("create_qa_chain", (llm,))
            .expect("建立 chain 的過程發生錯誤");

        let result = chain
            .call_method1("invoke", ("請問台灣最嚴重的颱風？",))
            .expect("推論的過程發生錯誤");

        println!("{}", result.to_string());
    });
    Ok(())
}
