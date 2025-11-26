use rand::{Rng, distributions};




fn gen_bad_name() -> String {
    rand::thread_rng().sample_iter(distributions::Alphanumeric).take(10).map(char::from).collect()
}

// ############################################################################
