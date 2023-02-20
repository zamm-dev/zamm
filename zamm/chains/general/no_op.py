from typing import Any, Dict, List

from langchain.chains.base import Chain


class NoOpChain(Chain):
    @property
    def input_keys(self) -> List[str]:
        """Input keys this chain expects."""
        return []

    @property
    def output_keys(self) -> List[str]:
        """Output keys this chain expects."""
        return []

    def _call(self, inputs: Dict[str, Any]) -> Dict[str, str]:
        return inputs
