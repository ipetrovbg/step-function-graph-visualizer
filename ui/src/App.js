import React from "react";

import AWSSfnGraph from "@tshepomgaga/aws-sfn-graph";
import "@tshepomgaga/aws-sfn-graph/index.css";
import TextArea from "./TextArea";
import GenerateButton from "./GenerateButton";

const SAMPLE_DATA = `stepFunctions:
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
            End: true`;

const App = () => {
  let url = "";

  if (process.env.NODE_ENV !== "development") {
    url =
      "https://6mdgghetvnrbd6bnvtqnrvgure0atymm.lambda-url.eu-central-1.on.aws/";
  } else {
    url = "http://localhost:9000/lambda-url/step_function_parser/";
  }

  const [ymlStr, setYmlStr] = React.useState(SAMPLE_DATA);
  const [aslData, setAslData] = React.useState({});
  const [error, setError] = React.useState(false);
  const getAslData = React.useCallback(async () => {
    fetch(url, {
      method: "POST",
      body: ymlStr,
    })
      .then((res) => {
        if (!res.ok) {
          return res.text().then((text) => {
            setError(true);
            setAslData({
              StartAt: `${text}`,
              States: {
                [`${text}`]: {
                  Type: "Task",
                  End: true,
                },
              },
            });
          });
        } else {
          return res.json();
        }
      })
      .then((data) => {
        if (data) {
          setError(false);
          setAslData(data);
        }
      })
      .catch((err) => {
        setError(true);
        setAslData({
          StartAt: `${err.message}`,
          States: {
            [`${err.message}`]: {
              Type: "Task",
              End: true,
            },
          },
        });
      });
  }, [url, ymlStr]);

  React.useEffect(() => {
    getAslData();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <div className="p-4 grid grid-flow-row-dense gap-4 grid-cols-11 ">
      <TextArea
        error={error}
        ymlStr={ymlStr}
        onChange={(str) => {
          setYmlStr(str);
        }}
      />
      <GenerateButton onClick={getAslData} />
      <div
        className={`rounded col-span-5 w-full p-1 h-full bg-gradient-to-r ${
          error
            ? "from-red-600 via-purple-700 to-rose-600"
            : "from-indigo-500 via-green-500 to-teal-500"
        }`}
      >
        <div className="rounded bg-slate-50 h-full w-full">
          <AWSSfnGraph data={aslData} onError={console.log} />
        </div>
      </div>
    </div>
  );
};

export default App;
