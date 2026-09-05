use tunes::prelude::*;


fn main()-> Result<()> {
    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    let quarter = comp.tempo().quarter_note();
    let eighth = comp.tempo().eighth_note();
    let sixteenth = comp.tempo().sixteenth_note();
    let halfnote = quarter * 2.0;

    comp.instrument("Treble",&Instrument::harpsichord())
        .notes(&[C5,C5,G5,G5,A5,A5,G5,G5,F5,F5,E5,E5,D5], quarter)
        .note(&[D5],eighth+sixteenth)
        .note(&[E5],sixteenth)
        .note(&[C5],halfnote);

    comp.instrument("Bass",&Instrument::harpsichord())
        .notes(&[C3,C4,E4,C4,F4,C4,E4,C4,D4,B3,C4,A3,F3,G3], quarter)
        .note(&[C3],halfnote);



    engine.play_mixer(&comp.into_mixer())?;
    Ok(())    
}
