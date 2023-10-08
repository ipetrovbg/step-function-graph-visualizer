import React from "react";

import AWSSfnGraph from "@tshepomgaga/aws-sfn-graph";
import "@tshepomgaga/aws-sfn-graph/index.css";
import TextArea from "./TextArea";

const SAMPLE_DATA = `
stepFunctions:
  validate: true
  stateMachines:
    DemoStateMachine:
      name: DemoStateMachine
      definition:
        Comment: Demo State Machine comment
        StartAt: FirstStep
        States:
          FirstStep:
            Type: Task
            Resource:
              Fn::GetAtt: [first, Arn]
            ResultPath: $.first
            Next: Second Step
          Second Step:
            Type: Task
            Resource:
              Fn::GetAtt: [second, Arn]
            Next: ChoiceStep
          ChoiceStep:
            Type: Choice
            Choices:
              - Variable: $.hasChoice
                IsPresent: true
                Next: SomeChoice 
              - Variable: $.hasChoice
                IsPresent: false
                Next: SomeChoice
            Default: NoChoice
          SomeChoice:
            Type: Task
            Resource:
              Fn::GetAtt: [someChoice, Arn]
            Next: NoChoice 
          NoChoice:
            Type: Pass
            End: true
`;

const App = () => {
  let url = "http://localhost:9000/lambda-url/step_function_parser/";

  if (process.env.NODE_ENV !== "development") {
    url = "production-url";
  }

  const [ymlStr, setYmlStr] = React.useState(SAMPLE_DATA);
  const [aslData, setAslData] = React.useState({});
  const [error, setError] = React.useState(false);
  const getAslData = React.useCallback(async () => {
    fetch(url, {
      method: "POST",
      mode: "cors",
      redirect: "follow",
      body: ymlStr,
    })
      .then((res) => {
        if (!res.ok) {
          return res.text().then((text) => {
            setError(text);
            // reporting errors directly into the graph itself
            return {
              StartAt: `${text}`,
              States: {
                [`${text}`]: {
                  Type: "Task",
                  End: true,
                },
              },
            };
          });
        } else {
          setError(false);
          return res.json();
        }
      })
      .then((data) => {
        if (data) {
          setAslData(data);
        }
      });
  }, [url, ymlStr]);

  React.useEffect(() => {
    console.log("useEffect");
    getAslData();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <div className="p-4 grid grid-flow-row-dense gap-4 grid-cols-11">
      <TextArea
        error={error}
        ymlStr={ymlStr}
        onChange={(str) => {
          setYmlStr(str);
        }}
      />
      <div className="flex justify-center self-center">
        <button
          onClick={() => getAslData()}
          className="w-full bg-indigo-600 rounded text-slate-50 p-4"
        >
          Generate Graph
        </button>
      </div>

      <div
        className={`col-span-5 rounded border-solid border-4 ${
          error ? "border-red-600" : " border-indigo-600"
        }`}
      >
        <AWSSfnGraph data={aslData} onError={console.log} />
      </div>
    </div>
  );
};

export default App;
