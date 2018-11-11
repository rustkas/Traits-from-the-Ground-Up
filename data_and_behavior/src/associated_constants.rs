trait SuperLog {
    const LABEL: Display;

    fn log(&self, f: &mut fmt::Formatter) -> fmt::Result;
}