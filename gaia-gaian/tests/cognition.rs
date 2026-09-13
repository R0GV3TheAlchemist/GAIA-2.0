use gaia_gaian::{g2g_send, Consent, DigitalMe, GaianError, MemoryTier, Persona, PersonalMemory};

fn adult() -> Consent {
    Consent {
        subject_is_self: true,
        age_years: 34,
        self_consent: true,
        parental_consent: false,
        health_opt_in: false,
    }
}

fn child() -> Consent {
    Consent {
        subject_is_self: true,
        age_years: 12,
        self_consent: true,
        parental_consent: true,
        health_opt_in: false,
    }
}

#[test]
fn digital_me_runs_offline_and_memory_wipes() {
    let reply = DigitalMe::ask(&adult(), "hello").unwrap();
    assert!(reply.contains("offline"));
    let mut mem = PersonalMemory::default();
    mem.put(MemoryTier::Episodic, "note");
    assert_eq!(mem.inspect().len(), 1);
    mem.wipe();
    assert!(mem.inspect().is_empty());
}

#[test]
fn under_16_cannot_enable_behavioral_learning() {
    assert_eq!(
        Persona::editable(&child()).unwrap_err(),
        GaianError::BehavioralLearningBlocked
    );
    let mut persona = Persona::editable(&adult()).unwrap();
    assert_eq!(
        persona.enable_learning(&child()).unwrap_err(),
        GaianError::BehavioralLearningBlocked
    );
}

#[test]
fn unsigned_g2g_is_rejected() {
    assert_eq!(g2g_send(false, "greet").unwrap_err(), GaianError::Unsigned);
    g2g_send(true, "greet").unwrap();
}
