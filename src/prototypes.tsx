import { NumberInput, TextInput } from "@mantine/core";
import { useForm, UseFormReturnType } from "@mantine/form";

interface FormASchema {
  name: string;
  age: number;
}

export function FormA() {
  const form = useForm<FormASchema>();
  return (
    <>
      <TextInput {...form.getInputProps("name")} />
      <FormB form={form} />
    </>
  );
}

interface FormBSchema {
  age: number;
}

function FormB<T extends FormBSchema>({
  form,
}: {
  form: UseFormReturnType<T>;
}) {
  return <NumberInput {...form.getInputProps("age")} />;
}
