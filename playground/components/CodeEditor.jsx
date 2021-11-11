import React from 'react';
import { useColorModeValue } from '@chakra-ui/react';
import dynamic from 'next/dynamic';
import '@uiw/react-textarea-code-editor/dist.css';

const CodeEditorArea = dynamic(
  () => import('@uiw/react-textarea-code-editor').then((mod) => mod.default),
  { ssr: false },
);

const CodeEditor = ({ code, setCode }) => {
  const backgroundColor = useColorModeValue('gray.100', 'gray.900');

  return (
    <CodeEditorArea
      value={code}
      language="clojure"
      placeholder="Write your code here"
      onChange={(e) => setCode(e.target.value)}
      height="100%"
      autoFocus
      style={{
        backgroundColor,
        flex: 1,
        fontSize: 16,
        fontFamily:
          'ui-monospace,SFMono-Regular,SF Mono,Consolas,Liberation Mono,Menlo,monospace',
      }}
    />
  );
};

export default CodeEditor;
