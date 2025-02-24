import { A } from "@solidjs/router";
import { Box, HStack, Heading } from "@hope-ui/solid";

const Navigation = () => {
  return (
    <Box
      as="nav"
      w="100%"
      py="$4"
      px="$6"
      bg="white"
      boxShadow="0 1px 2px 0 rgba(0, 0, 0, 0.05)"
    >
      <HStack justify="space-between" maxW="$container.xl" mx="auto">
        <Heading size="xl" color="$primary9">Solid Demo</Heading>
        <HStack spacing="$6">
          <A
            href="/"
            class="nav-link"
            style={{
              "text-decoration": "none",
              color: "var(--hope-colors-neutral12)",
              "font-weight": "500",
            }}
          >
            Home
          </A>
          <A
            href="/about"
            class="nav-link"
            style={{
              "text-decoration": "none",
              color: "var(--hope-colors-neutral12)",
              "font-weight": "500",
            }}
          >
            About
          </A>
          <A
            href="/projects"
            class="nav-link"
            style={{
              "text-decoration": "none",
              color: "var(--hope-colors-neutral12)",
              "font-weight": "500",
            }}
          >
            Projects
          </A>
        </HStack>
      </HStack>
    </Box>
  );
};

export default Navigation; 
