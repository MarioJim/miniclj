import React, { useState } from 'react';
import {
  Box,
  Divider,
  Grid,
  GridItem,
  useColorModeValue,
} from '@chakra-ui/react';
import Head from 'next/head';
import type { NextPage } from 'next';

import Header from '../components/Header';
import CodeEditor from '../components/CodeEditor';
import OutputTabs from '../components/OutputTabs';

const initialCode = `(defn factorial [n]
  (if (= n 0)
    1
    (* n (factorial (- n 1)))))

(println "The factorial of" 5 "is" (factorial 5))
`;

const Home: NextPage = () => {
  const bg = useColorModeValue('gray.50', 'gray.800');
  const [code, setCode] = useState(initialCode);

  return (
    <div style={{ maxHeight: '100vh' }}>
      <Head>
        <title>miniclj Playground</title>
      </Head>

      <Box minHeight="100vh" bg={bg} display="flex" flexDirection="column">
        <Header />
        <Grid templateColumns="1fr 1px 1fr">
          <GridItem>
            <CodeEditor code={code} setCode={setCode} />
          </GridItem>
          <GridItem>
            <Divider orientation="vertical" borderColor="gray.200" />
          </GridItem>
          <GridItem>
            <OutputTabs code={code} />
          </GridItem>
        </Grid>
      </Box>
    </div>
  );
};

export default Home;
