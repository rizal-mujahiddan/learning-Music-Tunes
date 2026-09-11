use tunes::prelude::*;

fn main() -> Result<()> {
    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    let quarter = comp.tempo().quarter_note();
    let eighth = comp.tempo().eighth_note();
    let sixteenth = comp.tempo().sixteenth_note();
    let thirtysecondth = sixteenth / 2.0;
    let halfnote = quarter * 2.0;

    // Treble
    comp.instrument("Treble", &Instrument::celesta())
        // Phrase A
        .notes(
            &[C5, C5, G5, G5, A5, A5, G5, G5, F5, F5, E5, E5, D5],
            quarter,
        )
        .note(&[D5], eighth + sixteenth)
        .note(&[E5], sixteenth)
        .note(&[C5], halfnote)

        // Phrase A repeat
        .notes(
            &[C5, C5, G5, G5, A5, A5, G5, G5, F5, F5, E5, E5, D5],
            quarter,
        )
        .note(&[D5], eighth + sixteenth)
        .note(&[E5], sixteenth)
        .note(&[C5], halfnote)

        // Phrase B with grace notes
        .notes(&[G5, G5, F5, F5, E5, E5, D5, D5, G5, G5, F5, F5,E5], quarter)
        .notes(&[F5,E5,D5], thirtysecondth)
        .note(&[F5], eighth+sixteenth-(3.0*thirtysecondth))
        .note(&[F5], sixteenth)

        // Phrase C with grace notes
        .notes(
            &[E5, D5, C5, C5, G5, G5, A5, A5, G5, G5, F5, F5, E5, E5,D5],
            quarter,
        )
        .notes(&[E5,D5,C5],thirtysecondth)
        .note(&[D5], eighth+sixteenth-(3.0*thirtysecondth))
        .note(&[C5],halfnote)
        // var 1
        .notes(&[D5,C5,B4,C5,B4,C5,B4,C5,A5,G5,FS5,G5,FS5,G5,FS5,G5
            ,GS5,A5,C6,B5,D6,C6,B5,A5,A5,G5,E6,D6,C6,B5,A5,G5,G5,F5,D6,C6,B5,A5,G5,F5,
            F5,E5,C6,B5,A5,G5,F5,E5],sixteenth)
        .notes(&[D5,A5,G5,B4], eighth)
        .note(&[C5],quarter).wait(quarter)
        // var 1-2
        .notes(&[D5,C5,B4,C5,B4,C5,B4,C5,A5,G5,FS5,G5,FS5,G5,FS5,G5
            ,GS5,A5,C6,B5,D6,C6,B5,A5,A5,G5,E6,D6,C6,B5,A5,G5,G5,F5,D6,C6,B5,A5,G5,F5,
            F5,E5,C6,B5,A5,G5,F5,E5],sixteenth)
        .notes(&[D5,A5,G5,B4], eighth)
        .note(&[C5],quarter).wait(quarter)
        // var 1-3
        .notes(&[A5,G5,FS5,G5,FS4,G5,A5,G5,G5,F5,E5,F5,E5,F5,G5,F5,
            F5,E5,DS5,E5,DS5,E5,F5,E5,E5,D5,CS5,D5,CS5,D5,E5,D5,
            A5,G5,FS5,G5,E6,C6,A5,G5,G5,F5,E5,F5,D6,B5,G5,F5,
            F5,E5,DS5,E5,C6,G5,F5,E5],sixteenth)
        .note(&[G5],eighth+sixteenth)
        .note(&[E5],sixteenth)
        .note(&[D5],quarter)
        .notes(&[D5,C5,B4,C5,B4,C5,B4,C5,A5,G5,FS5,G5,FS5,G5,FS5,G5,
            GS5,A5,C6,B5,D6,C6,B5,A5,
            A5,G5,E6,D6,C6,B5,A5,G5,
            G5,F5,D6,C6,B5,A5,G5,F5,
            F5,E5,C6,B5,A5,G5,F5,E5,
            ], sixteenth)
        .notes(&[D5,A5,G5,B4],quarter)
        .note(&[C5], quarter).wait(quarter)
        ;


    // Bass
    comp.instrument("Bass", &Instrument::celesta())
        .notes(
            &[C3, C4, E4, C4, F4, C4, E4, C4, D4, B3, C4, A3, F3, G3],
            quarter,
        )
        .note(&[C3], halfnote)
        .notes(
            &[C3, C4, E4, C4, F4, C4, E4, C4, D4, B3, C4, A3, F3, G3],
            quarter,
        )
        .note(&[C3], halfnote)
        .notes(
            &[E4, G3, D4, G3, C4, G3, B3, G3, E4, G3, D4, G3, C4],
            quarter,
        )
        .note(&[C4], eighth + sixteenth)
        .note(&[D4], sixteenth)
        .note(&[G3, C3], quarter)
        .notes(
            &[B3, C3, C4, E4, C4, F4, C4, E4, C4, D4, B3, C4, A3, F3, G3],
            quarter,
        )
        .note(&[C3], halfnote)
        // var 1
        .notes(&[C3,C4,E4,C4,F4,C4], quarter)
        .note(&[E4,C4],quarter)
        .wait(eighth+sixteenth)
        .note(&[CS4], sixteenth)
        .note(&[D4],quarter)
        .wait(eighth+sixteenth)
        .note(&[B3], sixteenth)
        .note(&[C4],quarter)
        .wait(eighth+sixteenth)
        .note(&[A3], sixteenth)
        .notes(&[F3,G3], quarter)
        .notes(&[C4,G3,E3,G3], eighth)
        //var 1-2 times
        .notes(&[C3,C4,E4,C4,F4,C4], quarter)
        .note(&[E4,C4],quarter)
        .wait(eighth+sixteenth)
        .note(&[CS4], sixteenth)
        .note(&[D4],quarter)
        .wait(eighth+sixteenth)
        .note(&[B3], sixteenth)
        .note(&[C4],quarter)
        .wait(eighth+sixteenth)
        .note(&[A3], sixteenth)
        .notes(&[F3,G3,C4,C3], quarter)
        // var 1-3
        .notes(&[E4,G3,D4,G3,C4,G3,F4,G3], quarter)
        .note(&[E4,G3],halfnote)
        .note(&[D4,G3],halfnote)
        .note(&[C4,G3],quarter).wait(eighth+sixteenth)
        .note(&[C4],sixteenth)
        .note(&[E4],eighth+sixteenth)
        .note(&[C4],sixteenth)
        .notes(&[B3,C3,C4,E4,C4,F4,C4],quarter)
        .note(&[C4,E4],sixteenth).wait(eighth+sixteenth).note(&[CS4], sixteenth)
        .note(&[D4],quarter).wait(eighth+sixteenth).note(&[B3], sixteenth)
        .note(&[C4],quarter).wait(eighth+sixteenth).note(&[A3], sixteenth)
        .notes(&[F3,G3,C4,C3],quarter)
        ;


        
    engine.play_mixer(&comp.into_mixer())?;
    Ok(())
}