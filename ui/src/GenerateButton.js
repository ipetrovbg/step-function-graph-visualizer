const GenerateButton = ({ onClick }) => {
  return (
    <div className="flex justify-center self-center">
      <button
        onClick={onClick}
        className="w-full bg-indigo-600 rounded text-slate-50 p-4"
      >
        Generate
      </button>
    </div>
  );
};

export default GenerateButton;
