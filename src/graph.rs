use ringbuf::Consumer;

pub trait Node {
    fn read(&mut self, buffer: &mut [f32]);
}

pub struct InputNode {
    consumer: Consumer<f32>,
}

impl InputNode {
    pub fn new(consumer: Consumer<f32>) -> Self {
        Self { consumer }
    }
}

impl Node for InputNode {
    fn read(&mut self, buffer: &mut [f32]) {
        if self.consumer.pop_slice(buffer) < buffer.len() {
            eprintln!("Input stream fell behind");
        }
    }
}

pub struct Graph<I, N>
where
    I: Node,
    N: Node,
{
    input: I,
    nodes: Vec<N>,
}

impl<I, N> Graph<I, N>
where
    I: Node,
    N: Node,
{
    pub fn new(input: I, nodes: Vec<N>) -> Self {
        Self { input, nodes }
    }
}

impl<I, N> Node for Graph<I, N>
where
    I: Node,
    N: Node,
{
    fn read(&mut self, buffer: &mut [f32]) {
        self.input.read(buffer);

        for node in &mut self.nodes {
            node.read(buffer);
        }
    }
}
