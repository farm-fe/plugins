import { VStack, Heading, Text, HStack, Avatar, Box } from "@hope-ui/solid";

const TeamMember = (props: { name: string; role: string; avatar: string }) => {
  return (
    <Box
      bg="white"
      p="$6"
      borderRadius="$lg"
      boxShadow="$sm"
      textAlign="center"
    >
      <Avatar
        size="2xl"
        name={props.name}
        src={props.avatar}
        mb="$4"
      />
      <Heading size="lg" mb="$2">{props.name}</Heading>
      <Text color="$neutral11">{props.role}</Text>
    </Box>
  );
};

const About = () => {
  return (
    <VStack spacing="$12" align="stretch">
      <VStack spacing="$4" textAlign="center">
        <Heading size="3xl" color="$primary9">About Us</Heading>
        <Text size="xl" maxW="$container.md" mx="auto" color="$neutral11">
          We're a passionate team dedicated to building amazing web experiences with SolidJS
        </Text>
      </VStack>

      <VStack spacing="$8" align="stretch">
        <Heading size="2xl" textAlign="center" color="$primary9">Our Mission</Heading>
        <Text size="lg" textAlign="center" maxW="$container.lg" mx="auto" color="$neutral11">
          To create beautiful, performant, and accessible web applications that make a difference in people's lives.
          We believe in the power of modern web technologies and their ability to transform the digital landscape.
        </Text>
      </VStack>

      <VStack spacing="$8" align="stretch">
        <Heading size="2xl" textAlign="center" color="$primary9">Our Team</Heading>
        <HStack wrap="wrap" spacing="$6" justify="center">
          <TeamMember
            name="Alex Johnson"
            role="Lead Developer"
            avatar="https://i.pravatar.cc/150?u=1"
          />
          <TeamMember
            name="Sarah Chen"
            role="UI/UX Designer"
            avatar="https://i.pravatar.cc/150?u=2"
          />
          <TeamMember
            name="Mike Wilson"
            role="Product Manager"
            avatar="https://i.pravatar.cc/150?u=3"
          />
        </HStack>
      </VStack>
    </VStack>
  );
};

export default About; 
