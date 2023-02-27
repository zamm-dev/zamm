from langchain.prompts.prompt import PromptTemplate

FOLLOW_TUTORIAL_PROMPT = PromptTemplate(
    input_variables=[],
    template="You follow the tutorial located at: ",
)

FOLLOW_TUTORIAL_LOGGER = PromptTemplate(
    input_variables=["tutorial"],
    template="\nYou followed the tutorial at {tutorial}",
)
