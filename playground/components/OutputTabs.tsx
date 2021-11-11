import React, { useEffect, useState } from 'react';
import {
  Alert,
  AlertDescription,
  AlertIcon,
  AlertTitle,
  Box,
  Tab,
  TabList,
  TabPanel,
  TabPanels,
  Tabs,
} from '@chakra-ui/react';
import dynamic from 'next/dynamic';

type ResultTabProps =
  | { status: 'ok'; output: string }
  | { status: 'error'; error: string };

const ResultTab = (props: ResultTabProps): JSX.Element => (
  <>
    {props.status === 'ok' && <pre>{props.output}</pre>}
    {props.status === 'error' && (
      <Alert
        status="warning"
        flexDirection="column"
        alignItems="center"
        justifyContent="center"
        textAlign="center"
      >
        <AlertIcon boxSize="40px" mr={0} />
        <AlertTitle mt={4} mb={1} fontSize="lg">
          Error
        </AlertTitle>
        <AlertDescription maxWidth="sm">{props.error}</AlertDescription>
      </Alert>
    )}
  </>
);

interface MinicljOutputState {
  ast: ResultTabProps;
  compile: ResultTabProps;
  run: ResultTabProps;
}

const MinicljOutput = dynamic(
  async () => {
    const miniclj = await import('miniclj-wasm');

    const MinicljOutput = ({ code }: { code: string }): JSX.Element => {
      const [state, setState] = useState<MinicljOutputState>({
        ast: { status: 'ok', output: '' },
        compile: { status: 'ok', output: '' },
        run: { status: 'ok', output: '' },
      });

      useEffect(() => {
        (window as any).minicljoutput = '';

        const newState = {
          ast: miniclj.ast(code),
          compile: miniclj.compile(code),
          run: miniclj.run(code),
        };
        if (newState.run.status === 'ok') {
          newState.run.output = (window as any).minicljoutput as string;
        }

        setState(newState);
      }, [code]);

      return (
        <Tabs isFitted variant="enclosed">
          <TabList mb="1em">
            <Tab>AST</Tab>
            <Tab>Bytecode</Tab>
            <Tab>Execution</Tab>
          </TabList>
          <TabPanels overflow="auto" height="80vh">
            <TabPanel>
              <ResultTab {...state.ast} />
            </TabPanel>
            <TabPanel>
              <ResultTab {...state.compile} />
            </TabPanel>
            <TabPanel>
              <ResultTab {...state.run} />
            </TabPanel>
          </TabPanels>
        </Tabs>
      );
    };

    return MinicljOutput;
  },
  { ssr: false },
);

interface OutputTabsProps {
  code: string;
}

const OutputTabs = ({ code }: OutputTabsProps): JSX.Element => {
  return (
    <Box flex="1" padding="15px">
      <MinicljOutput code={code} />
    </Box>
  );
};

export default OutputTabs;
