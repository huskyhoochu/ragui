import React, { useRef, useEffect, useState } from "react";
import ReactDOM from "react-dom";

const App: React.FC = () => {
  const [html, setHtml] = useState<string>('');
  const parserRef = useRef<((input: string) => string) | undefined>(undefined);

  useEffect(() => {
    async function fetchWasm() {
      const parser = await import('../pkg');
      parserRef.current = parser.parse;
    }

    fetchWasm().catch(e => console.error(e));
  }, []);

  const onChange = (e: React.BaseSyntheticEvent) => {
    const parsed = parserRef.current ? parserRef.current(e.currentTarget.value): '';
    setHtml(parsed);
  }

  return (
    <div>
      <textarea name="input" id="input" cols={60} rows={30} onChange={onChange} />
      <div
        dangerouslySetInnerHTML={{
          __html: html,
        }}
      />
    </div>
  );
};
const root = document.getElementById("root");

ReactDOM.render(<App />, root);
