import warnings
from typing import Generator, Iterator

from llama_cpp import Llama


def inference(llm: Llama, prompt: str, **kwargs) -> str:
    if "stream" in kwargs and kwargs["stream"]:
        warnings.warn(Warning("發現 stream 在參數中，預設為單一回應而非串流回應。"))
        del kwargs["stream"]

    response = llm(prompt, **kwargs)

    try:
        assert isinstance(response, dict), "不是正常的回應"
        return "\n".join(ch["text"] for ch in response["choices"])
    except AssertionError as e:
        raise e


def streaming(llm: Llama, prompt: str, **kwargs) -> Generator[str, None, None]:
    if "stream" in kwargs and kwargs["stream"]:
        warnings.warn(Warning("發現 stream 在參數中，預設為串流回應而非單一回應。"))
        del kwargs["stream"]

    streaming = llm(prompt, **kwargs)

    try:
        assert isinstance(streaming, Iterator), "不是 streaming"
        for res in streaming:
            assert isinstance(res, dict), "不是正常的回應"
            result = "\n".join(ch["text"] for ch in res["choices"])
            yield result
    except AssertionError as e:
        raise e


def quantize_model(model_path: str, **kwargs) -> Llama:
    return Llama(model_path=model_path, **kwargs)  # type: ignore
