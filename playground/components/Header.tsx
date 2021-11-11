import React from 'react';
import {
  Box,
  Button,
  Flex,
  Heading,
  HStack,
  useColorMode,
  useColorModeValue,
} from '@chakra-ui/react';
import Link from 'next/link';
import { CgExternal, CgMoon, CgSun } from 'react-icons/cg';

const Header = (): JSX.Element => {
  const { colorMode, toggleColorMode } = useColorMode();
  const bg = useColorModeValue('gray.100', 'gray.900');
  const borderColor = useColorModeValue('gray.300', 'gray.700');

  return (
    <Flex
      as="header"
      justifyContent="space-between"
      h="56px"
      px={[2, 2, 5]}
      py="2"
      bg={bg}
      borderBottomWidth="1px"
      borderBottomColor={borderColor}
    >
      <Heading size="lg">miniclj</Heading>
      <HStack spacing="4">
        <Button variant="ghost" onClick={toggleColorMode}>
          {colorMode === 'dark' ? <CgMoon /> : <CgSun />}
        </Button>
        <Link href="https://github.com/MarioJim/miniclj" passHref>
          <Box
            display="flex"
            alignItems="center"
            style={{ gap: '4px', cursor: 'pointer' }}
          >
            View the repo on GitHub
            <CgExternal />
          </Box>
        </Link>
      </HStack>
    </Flex>
  );
};

export default Header;
