import { Alert, Container, MantineProvider } from "@mantine/core";
import { IconInfoCircle } from "@tabler/icons-react";

export default function ErrorPage({ error }: { error: Error }) {
  const icon = <IconInfoCircle />;
  return (
    <MantineProvider defaultColorScheme="dark">
      <Container>
        <Alert variant="light" color="red" title="Error occurred" icon={icon}>
          {error.message}
        </Alert>
      </Container>
    </MantineProvider>
  );
}
