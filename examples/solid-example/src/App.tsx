import { Router, Routes, Route } from "@solidjs/router";
import { HopeProvider, Container, VStack } from "@hope-ui/solid";
import Navigation from "./components/Navigation";
import Home from "./pages/Home";
import About from "./pages/About";
import Projects from "./pages/Projects";

function App() {
  return (
    <HopeProvider>
      <Router>
        <VStack minH="100vh" minW="100vw" bg="#ffbbaa">
          <Navigation />
          <Container maxW="$container.xl" p="$6" flex={1}>
            <Routes>
              <Route path="/" component={Home} />
              <Route path="/about" component={About} />
              <Route path="/projects" component={Projects} />
            </Routes>
          </Container>
        </VStack>
      </Router>
    </HopeProvider>
  );
}

export default App;
