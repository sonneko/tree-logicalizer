// treelogicalizer-ui/src/App.tsx
import { useState, useEffect } from 'react';
import init, { CircuitSimulator } from './wasm/tree_logicalizer_core';

function App() {
  const [output, setOutput] = useState('Initializing Wasm...');
  
  // Day 2 テスト用の拡張DSL
  const dslCode = `
    module FullAdder(In A, In B, In Cin, Out Sum, Out Cout) {}
    module GenericAdder<N>(In A[N], In B[N], Out S[N]) {
        wire clk;
        bus Carry[N];
        bus Data[16];
    }
  `;

  useEffect(() => {
    init()
      .then(() => {
        try {
          // 拡張DSLでCircuitSimulatorを初期化
          const simulator = new CircuitSimulator(dslCode);
          setOutput(simulator.get_info()); // 新しいget_infoを呼び出し
        } catch (error) {
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
      <h1>TreeLogicalizer Day 2</h1>
      <p>Wasm Output: <strong>{output}</strong></p>
      {/* DSLエディタの代わり */}
      <pre style={{ border: '1px solid #ccc', padding: '10px' }}>{dslCode}</pre>
    </div>
  );
}

export default App;