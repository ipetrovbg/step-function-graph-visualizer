import React from "react";

const TextArea = ({ ymlStr, error, onChange }) => {
  return (
    <div
      className={`rounded col-span-5 w-full p-1 h-full bg-gradient-to-r ${
        error
          ? "from-rose-600 via-purple-700 to-red-600"
          : "from-teal-500 via-green-500 to-indigo-500"
      }`}
    >
      <div className="w-full h-full rounded bg-slate-50">
        <textarea
          className="h-full w-full"
          value={ymlStr}
          onChange={(val) => onChange(val.target.value)}
        />
      </div>
    </div>
  );
};

export default TextArea;
