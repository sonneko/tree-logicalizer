// treelogicalizer-ui/src/App.tsx
import { useState, useEffect } from 'react';
// Wasmモジュールをインポート
import init, { CircuitSimulator } from './wasm/tree_logicalizer_core';

function App() {
  const [output, setOutput] = useState('Initializing Wasm...');
  const dslCode = 'module MyFirstCircuit {}'; // テスト用の最小DSL

  useEffect(() => {
    init()
      .then(() => {
        try {
          // 最小のDSLでCircuitSimulatorを初期化
          const simulator = new CircuitSimulator(dslCode);
          setOutput(simulator.greet()); // Wasmのgreet関数を呼び出し
          // 実際のインスタンスを状態に保持すべきだが、Day 1は確認まで
        } catch (error) {
          // パースエラーなどがここでキャッチされる
          setOutput(`Initialization Error: ${error}`);
          console.error(error);
        }
      })
      .catch(e => {
        setOutput(`Failed to load Wasm: ${e}`);
        console.error(e);
      });
  }, []);

  return (
    <div style={{ padding: '20px' }}>
      <h1>TreeLogicalizer Day 1</h1>
      <p>DSL Code: <code>{dslCode}</code></p>
      <p>Wasm Output: <strong>{output}</strong></p>
      {/* Day 2以降でCodeEditorとCircuitViewerを配置 */}
    </div>
  );
}

export default App;