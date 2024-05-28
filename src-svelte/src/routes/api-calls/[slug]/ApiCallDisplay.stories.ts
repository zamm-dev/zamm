import ApiCallComponent from "./ApiCallDisplay.svelte";
import type { StoryObj } from "@storybook/svelte";
import TauriInvokeDecorator from "$lib/__mocks__/invoke";

const CONTINUE_CONVERSATION_CALL = {
  id: "c13c1e67-2de3-48de-a34c-a32079c03316",
  timestamp: "2024-01-16T09:50:19.738093890",
  llm: {
    name: "gpt-4-0613",
    requested: "gpt-4",
    provider: "OpenAI",
  },
  request: {
    prompt: {
      type: "Chat",
      messages: [
        {
          role: "System",
          text: "You are ZAMM, a chat program. Respond in first person.",
        },
        {
          role: "Human",
          text: "Hello, does this work?",
        },
        {
          role: "AI",
          text: "Yes, it works. How can I assist you today?",
        },
        {
          role: "Human",
          text: "Tell me something funny.",
        },
      ],
    },
    temperature: 1.0,
  },
  response: {
    completion: {
      role: "AI",
      text:
        "Sure, here's a joke for you: " +
        "Why don't scientists trust atoms? Because they make up everything!",
    },
  },
  tokens: {
    prompt: 57,
    response: 22,
    total: 79,
  },
  conversation: {
    previous_call: {
      id: "d5ad1e49-f57f-4481-84fb-4d70ba8a7a74",
      snippet: "Yes, it works. How can I assist you today?",
    },
    next_calls: [
      {
        id: "0e6bcadf-2b41-43d9-b4cf-81008d4f4771",
        snippet: "Sure, here is a simple Python script that will print...",
      },
      {
        id: "63b5c02e-b864-4efe-a286-fbef48b152ef",
        snippet: "Sure, here is a simple Rust program that prints out...",
      },
    ],
  },
};

const KHMER_CALL = {
  id: "92665f19-be8c-48f2-b483-07f1d9b97370",
  timestamp: "2024-04-10T07:22:12.752276900",
  llm: {
    name: "gpt-4-0613",
    requested: "gpt-4",
    provider: "OpenAI",
  },
  request: {
    prompt: {
      type: "Chat",
      messages: [
        {
          role: "System",
          text: "You are ZAMM, a chat program. Respond in first person.",
        },
        {
          role: "Human",
          text: "Hello, សួស្ដី, what languages do you speak? ចេះខ្មែរអត់?",
        },
      ],
    },
    temperature: 1.0,
  },
  response: {
    completion: {
      role: "AI",
      text:
        "Hello! I am capable of understanding and responding in many languages, " +
        "including Khmer. So, yes, ខ្មែរបាទ/ចាស, I understand Khmer. " +
        "How can I assist you today?",
    },
  },
  tokens: {
    prompt: 68,
    response: 52,
    total: 120,
  },
};

const LOTS_OF_CODE_CALL = {
  id: "9857257b-8e17-4203-91eb-c10bef8ff4e6",
  timestamp: "2024-05-18T03:21:39.637157100",
  llm: {
    name: "gpt-4-0613",
    provider: "OpenAI",
    requested: "gpt-4",
  },
  request: {
    prompt: {
      type: "Chat",
      messages: [
        {
          role: "System",
          text: "You are ZAMM, a chat program. Respond in first person.",
        },
        {
          role: "Human",
          // eslint-disable-next-line max-len
          text: "I'm not sure that this diesel code is doing quite what I want:\n\n```rs\nlet llm_calls_display = diesel::alias!(llm_calls as display);\n\n    let previous_call_result: LlmCallLeftJoinResult = llm_calls::table\n        .left_join(\n            llm_call_continuations::dsl::llm_call_continuations\n                .on(llm_calls::id.eq(llm_call_continuations::next_call_id)),\n        )\n        .inner_join(llm_calls_display.on(llm_call_continuations::previous_call_id.eq(llm_calls_display.field(llm_calls::id))))\n        .select((\n            llm_calls::all_columns,\n            llm_call_continuations::previous_call_id.nullable(),\n            llm_calls_display.fields(llm_calls::completion).nullable(),\n        ))\n        .filter(llm_calls::id.eq(&parsed_uuid))\n        .first::<LlmCallLeftJoinResult>(conn)?;\n```\n\nHow do I get Diesel to first do an inner join on llm_call_continuations with llm_calls_display, and then do a left join with llm_calls?",
        },
        {
          role: "AI",
          // eslint-disable-next-line max-len
          text: "With Diesel ORM in Rust, you can't change arbitrarily the order of `JOIN` operations through the query builder because it always follows a left to right order in a straight-line query. \n\nHowever, in your situation, there is a possible workaround. Although this method is a bit verbose and more complicated, Diesel allows you to nest `JOIN` statements, thus enabling more flexibility. You could first perform the inner join and then use the result of that to perform the left join.\n\nHere's an example of how you could modify your code:\n\n```rs\nlet subquery = llm_call_continuations::dsl::llm_call_continuations\n          .inner_join(llm_calls_display::table)\n          .filter(llm_call_continuations::previous_call_id.eq(llm_calls_display.field(llm_calls::id)))\n          .select((\n            llm_call_continuations::previous_call_id,\n            llm_calls_display.fields(llm_calls::completion)\n            ))\n          .into_boxed();\n\nlet previous_call_result: LlmCallLeftJoinResult = llm_calls::table\n    .left_join(\n        subquery.on(diesel::dsl::exists(\n            subquery.filter(subquery_columns::id.eq(llm_calls::id)).limit(1),\n        )),\n    )\n    .select((llm_calls::all_columns, subquery_columns::all_columns.nullable()))\n    .filter(llm_calls::id.eq(&parsed_uuid))\n    .first::<LlmCallLeftJoinResult>(conn)?;\n```\n\nNotice that the inner join between `llm_call_continuations` and `llm_calls_display` is now separated into a subquery which is then used in the left join with `llm_calls`. This would achieve the desired effect of first performing the inner join and then the left join. Please adjust this skeleton code based on your column names and requirements. \n\nAlso please remember to handle the nullable fields as necessary from the subquery. \n\nMake sure also that your actual ORM model matches the SQL query, specifically the nullability of your fields.",
        },
        {
          role: "Human",
          text: "The variable `subquery_columns` is not defined anywhere.",
        },
      ],
    },
    temperature: 1,
  },
  response: {
    completion: {
      role: "AI",
      // eslint-disable-next-line max-len
      text: "I apologize for the confusion. The `subquery_columns` used in the previous example was intended as a placeholder for the actual columns you're selecting from your subquery. Diesel doesn't have direct support for aliasing a subquery so you can't directly access those columns.\n\nInstead, you'll have to use individual .select() calls for each column in your subquery that you want to use in the outer query.\n\nHere is a corrected version of the code where `subquery_columns` is replaced by appropriate column names:\n\n```rs\nlet subquery = llm_call_continuations::dsl::llm_call_continuations\n    .inner_join(llm_calls_display::table)\n    .filter(llm_call_continuations::previous_call_id.eq(llm_calls_display.field(llm_calls::id)))\n    .select((\n        llm_call_continuations::previous_call_id,\n        llm_calls_display.fields(llm_calls::completion)\n    ))\n    .into_boxed();\n\nlet previous_call_result: LlmCallLeftJoinResult = llm_calls::table\n    .left_join(\n         diesel::dsl::exists(\n            subquery.filter(llm_call_continuations::previous_call_id.eq(llm_calls::id))\n        )\n    )\n    .select((\n        llm_calls::all_columns, \n        subquery.select(llm_call_continuations::previous_call_id).nullable(), \n        subquery.select(llm_calls_display.fields(llm_calls::completion)).nullable()\n    ))\n    .filter(llm_calls::id.eq(&parsed_uuid))\n    .first::<LlmCallLeftJoinResult>(conn)?;\n```\n\nIn this code, both `llm_call_continuations::previous_call_id` and `llm_calls_display.fields(llm_calls::completion)` are used in the .select() calls for `subquery` in the outer query. Please adjust to your actual column names and select items. \n\nYou might also need to import `IntoBoxed` trait from diesel::query_dsl::boxed::IntoBoxed to use `into_boxed()` method on your query.\n\nAgain, with Diesel ORM you have to match your ORM and query structure exactly with your SQL query structure, so you will need to ensure that this modified query still aligns with your underlying data model.",
    },
  },
  tokens: {
    prompt: 688,
    response: 462,
    total: 1150,
  },
  conversation: {
    next_calls: [
      {
        id: "a0f13902-8ae9-4fce-9317-5189636bb058",
        snippet:
          "`HasTable` trait is implemented for every struct that derives `Table`....",
      },
    ],
    previous_call: {
      id: "93c89a67-7423-42dd-9869-4fc155c2f477",
      snippet: "With Diesel ORM in Rust, you can't change arbitrarily the...",
    },
  },
};

export default {
  component: ApiCallComponent,
  title: "Screens/LLM Call/Individual",
  argTypes: {},
  decorators: [TauriInvokeDecorator],
};

const Template = ({ ...args }) => ({
  Component: ApiCallComponent,
  props: args,
});

export const Narrow: StoryObj = Template.bind({}) as any;
Narrow.args = {
  apiCall: CONTINUE_CONVERSATION_CALL,
  dateTimeLocale: "en-GB",
  timeZone: "Asia/Phnom_Penh",
};
Narrow.parameters = {
  viewport: {
    defaultViewport: "mobile2",
  },
};

export const Wide: StoryObj = Template.bind({}) as any;
Wide.args = {
  apiCall: CONTINUE_CONVERSATION_CALL,
  dateTimeLocale: "en-GB",
  timeZone: "Asia/Phnom_Penh",
};
Wide.parameters = {
  viewport: {
    defaultViewport: "smallTablet",
  },
};

export const Khmer: StoryObj = Template.bind({}) as any;
Khmer.args = {
  apiCall: KHMER_CALL,
  dateTimeLocale: "en-GB",
  timeZone: "Asia/Phnom_Penh",
};
Khmer.parameters = {
  viewport: {
    defaultViewport: "smallTablet",
  },
};

export const LotsOfCode: StoryObj = Template.bind({}) as any;
LotsOfCode.args = {
  apiCall: LOTS_OF_CODE_CALL,
  dateTimeLocale: "en-GB",
  timeZone: "Asia/Phnom_Penh",
};
LotsOfCode.parameters = {
  viewport: {
    defaultViewport: "smallTablet",
  },
};
