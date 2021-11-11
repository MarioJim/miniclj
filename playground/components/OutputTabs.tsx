import React, { useEffect, useRef, useState } from 'react';
import { Box, Tab, TabList, TabPanel, TabPanels, Tabs } from '@chakra-ui/react';
import dynamic from 'next/dynamic';

interface MinicljOutputProps {
  operation: 'ast' | 'compile' | 'run';
  code: string;
}

type MinicljResult =
  | { status: 'loading' }
  | { status: 'ok'; output: string }
  | { status: 'error'; error: string };

const MinicljOutput = dynamic(
  async () => {
    const miniclj = await import('miniclj-wasm');
    return function MinicljOutput({ operation, code }: MinicljOutputProps) {
      const [result, setResult] = useState<MinicljResult>({
        status: 'loading',
      });

      useEffect(() => {
        setResult(miniclj[operation](code));
      }, [operation, code]);

      if (result.status === 'loading') return <p>loading</p>;
      if (result.status === 'ok') return <pre>{result.output}</pre>;
      return <p>{result.error}</p>;
    };
  },
  { ssr: false },
);

interface OutputTabsProps {
  code: string;
}

const OutputTabs = ({ code }: OutputTabsProps): JSX.Element => {
  const outputDiv = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (outputDiv.current) {
      outputDiv.current.innerHTML = '';
    }
  }, [code]);

  return (
    <Box flex="1" padding="15px">
      <Tabs isFitted variant="enclosed">
        <TabList mb="1em">
          <Tab>AST</Tab>
          <Tab>Bytecode</Tab>
          <Tab>Execution</Tab>
        </TabList>
        <TabPanels overflow="auto" height="80vh">
          <TabPanel>
            <MinicljOutput operation="ast" code={code} />
          </TabPanel>
          <TabPanel>
            <MinicljOutput operation="compile" code={code} />
          </TabPanel>
          <TabPanel>
            <MinicljOutput operation="run" code={code} />
            <div ref={outputDiv} id="output"></div>
          </TabPanel>
        </TabPanels>
      </Tabs>
    </Box>
  );
};

export default OutputTabs;
