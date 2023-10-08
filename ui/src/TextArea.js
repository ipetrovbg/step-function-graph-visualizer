import React from "react";

const TextArea = ({ ymlStr, error, onChange }) => {
  return (
    <div
      className={
        error
          ? "w-full col-span-5 rounded border-solid border-4 border-red-600"
          : "w-full col-span-5 rounded border-solid border-4 border-indigo-600"
      }
    >
      <textarea
        className="h-full w-full"
        value={ymlStr}
        onChange={(val) => onChange(val.target.value)}
      />
    </div>
  );
};

export default TextArea;
