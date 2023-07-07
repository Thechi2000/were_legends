export function Button({ className, ...props }: any) {
  return (
    <div className={"flex justify-center items-center gap-2 border-2 border-sky-400 rounded-md p-1 " + className}>
      <i className="arrow-right" />
      <button {...props}></button>
      <i className="arrow-left" />
    </div>
  );
}
