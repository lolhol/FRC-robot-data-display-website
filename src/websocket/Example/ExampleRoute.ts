interface InputData {
  a: number;
  b: boolean;
}

export default async function ExampleRoute(data: InputData) {
  console.log("Example Route: ", data);
}
