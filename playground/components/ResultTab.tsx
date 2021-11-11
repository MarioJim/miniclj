import React from 'react';
import {
  Alert,
  AlertDescription,
  AlertIcon,
  AlertTitle,
} from '@chakra-ui/react';

type ResultTabProps =
  | { status: 'ok'; output: string }
  | { status: 'error'; error: string };

const ResultTab = (
  props: React.PropsWithChildren<ResultTabProps>,
): JSX.Element => (
  <>
    {props.status === 'ok' && (
      <>
        <pre>{props.output}</pre>
        {props.children}
      </>
    )}
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

export default ResultTab;
