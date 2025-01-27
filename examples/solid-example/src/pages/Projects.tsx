import { VStack, Heading, Text, SimpleGrid, Box, Badge, HStack } from "@hope-ui/solid";

const Project = (props: {
  title: string;
  description: string;
  tags: string[];
  link: string;
}) => {
  return (
    <Box
      as="a"
      href={props.link}
      target="_blank"
      rel="noopener noreferrer"
      bg="white"
      p="$6"
      borderRadius="$lg"
      boxShadow="$sm"
      _hover={{
        transform: "translateY(-4px)",
        transition: "transform 0.2s",
        textDecoration: "none",
      }}
    >
      <Heading size="lg" mb="$2" color="$primary9">
        {props.title}
      </Heading>
      <Text color="$neutral11" mb="$4">
        {props.description}
      </Text>
      <HStack spacing="$2" wrap="wrap">
        {props.tags.map((tag) => (
          <Badge colorScheme="primary" key={tag}>{tag}</Badge>
        ))}
      </HStack>
    </Box>
  );
};

const Projects = () => {
  const projects = [
    {
      title: "E-commerce Platform",
      description:
        "A modern e-commerce platform built with SolidJS and GraphQL",
      tags: ["SolidJS", "GraphQL", "TypeScript"],
      link: "https://github.com/example/ecommerce",
    },
    {
      title: "Task Management App",
      description:
        "A beautiful and intuitive task management application",
      tags: ["SolidJS", "Hope UI", "Firebase"],
      link: "https://github.com/example/tasks",
    },
    {
      title: "Weather Dashboard",
      description:
        "Real-time weather information with beautiful visualizations",
      tags: ["SolidJS", "D3.js", "API"],
      link: "https://github.com/example/weather",
    },
    {
      title: "Blog Platform",
      description:
        "A performant and SEO-friendly blog platform",
      tags: ["SolidJS", "MDX", "SEO"],
      link: "https://github.com/example/blog",
    },
  ];

  return (
    <VStack spacing="$8" align="stretch">
      <VStack spacing="$4" textAlign="center">
        <Heading size="3xl" color="$primary9">Our Projects</Heading>
        <Text size="xl" maxW="$container.md" mx="auto" color="$neutral11">
          Check out some of our amazing projects built with SolidJS
        </Text>
      </VStack>

      <SimpleGrid columns={{ "@initial": 1, "@md": 2 }} gap="$6">
        {projects.map((project) => (
          <Project {...project} key={project.title} />
        ))}
      </SimpleGrid>
    </VStack>
  );
};

export default Projects;

