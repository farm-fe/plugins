import { VStack, Heading, Text, SimpleGrid, Box, Icon } from "@hope-ui/solid";

const Feature = (props: { title: string; description: string }) => {
  return (
    <Box
      bg="white"
      p="$6"
      borderRadius="$lg"
      boxShadow="$sm"
      _hover={{ transform: "translateY(-4px)", transition: "transform 0.2s" }}
    >
      <Heading size="lg" mb="$2">{props.title}</Heading>
      <Text color="$neutral11">{props.description}</Text>
      1231321213123qweqweqw22222222
    </Box>
  );
};

const Home = () => {
  return (
    <VStack spacing="$8" align="stretch">
      <VStack spacing="$4" textAlign="center" py="$12">
        <Heading size="4xl" color="$primary9">
          Welcome to Solid
        </Heading>
        <Text size="xl" maxW="$container.md" mx="auto" color="$neutral11">
          A modern web framework that lets you build user interfaces with simple and reactive components
        </Text>
      </VStack>

      <SimpleGrid columns={{ "@initial": 1, "@md": 3 }} gap="$6">
        <Feature
          title="Reactive"
          description="Fine-grained reactivity means your app is optimized out of the box."
        />
        <Feature
          title="Performant"
          description="Top ranked in performance benchmarks, Solid is built for speed."
        />
        <Feature
          title="Pragmatic"
          description="Write code that's easy to understand and maintain with simple primitives."
        />
      </SimpleGrid>
    </VStack>
  );
};

export default Home; 
