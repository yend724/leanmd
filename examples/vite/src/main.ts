import { markdownToJSONPretty } from '../../../leanmd/pkg/leanmd';

// DOM要素を取得する関数を分離
const getElement = <T extends HTMLElement>(selector: string): T => {
  const element = document.querySelector<T>(selector);
  if (!element) {
    throw new Error(`${selector} not found`);
  }
  return element;
};

const renderAst = (markdown: string) => {
  const astOutput = getElement<HTMLTextAreaElement>('#ast-output');
  const jsonAst = markdownToJSONPretty(markdown);
  astOutput.textContent = JSON.stringify(JSON.parse(jsonAst), null, 2);
};

const run = () => {
  try {
    const markdownInput = getElement<HTMLTextAreaElement>('#markdown-input');

    // 初期レンダリング
    renderAst(markdownInput.value);

    // イベントリスナーの設定
    markdownInput.addEventListener('input', () => {
      renderAst(markdownInput.value);
    });
  } catch (error) {
    console.error('Initialization error:', error);
  }
};

run();
