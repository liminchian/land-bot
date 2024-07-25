from langchain_community.llms.llamacpp import LlamaCpp
from langchain_core.language_models.llms import LLM
from langchain_core.runnables import RunnablePassthrough
from langchain.prompts import PromptTemplate
from langchain_core.output_parsers.string import StrOutputParser

def create_qa_chain(llm: LLM):
    prompt = PromptTemplate.from_template("""
請依據問題來回答，並以三點以內的觀點來結構化輸出的內容，不需要額外說明：
{question}
""")
    chain = {"question": RunnablePassthrough()} | prompt | llm | StrOutputParser()
    return chain

def create_json_response_chain(llm: LLM):
    ...

def create_openai_chat_chain(llm: LLM):
    ...

def create_rag_chain(llm: LLM):
    ...

def load_gguf_model(model_path: str) -> LlamaCpp:
    return LlamaCpp(
        model_path=model_path,
        n_ctx=512,
        max_tokens=2048,
        n_gpu_layers=12,
        verbose=False,
        stop=["<|eot_id|>"]
    ) # type: ignore

