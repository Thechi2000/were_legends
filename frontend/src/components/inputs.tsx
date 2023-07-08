export function Button({ children, className, ...props }: any) {
  return (
    <div
      className={
        "flex justify-center items-center gap-2 border-2 border-sky-400 rounded-md p-1 cursor-pointer " +
        className
      }
      {...props}
    >
      <i className="arrow-right" />
      {children}
      <i className="arrow-left" />
    </div>
  );
}
