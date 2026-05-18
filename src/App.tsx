import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import buildInfoStatic from "./build-info.json";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [buildInfo, setBuildInfo] = useState<any | null>(null);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  useEffect(() => {
    invoke("get_build_info")
      .then((info: any) => setBuildInfo(info))
      .catch(() => setBuildInfo(null));
  }, []);

  return (
    <main className="container">
      <h1>Welcome to Tauri + React next version XXX breaking XXX WWW</h1>

      <div className="row">
        <a href="https://vite.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
      <section>
        <h3>Frontend build info (static)</h3>
        <pre>{JSON.stringify(buildInfoStatic, null, 2)}</pre>
      </section>
      <section>
        <h3>Runtime build info (from Rust)</h3>
        <pre>{buildInfo ? JSON.stringify(buildInfo, null, 2) : "(no runtime info)"}</pre>
      </section>
    </main>
  );
}

export default App;
