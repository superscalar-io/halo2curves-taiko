#[cfg(test)]
mod test {
    use crate::bn256::{Fq, Fr};
    use crate::utils::{
        read_field_from_file, read_fields_from_file, write_field_into_file, write_fields_into_file,
    };
    use pasta_curves::arithmetic::FieldExt;
    use std::fs::File;
    use std::io::{stdout, Read, Write};

    const fr_file_name: &str = "./fr_test_file.bin";
    const fq_file_name: &str = "./fq_test_file.bin";

    #[test]
    fn test_io_fr() {
        let fr = Fr::one();
        println!("{:?}", fr.0);
        println!("{:?}", fr);

        let mut file = File::create(fr_file_name).unwrap();

        file.write(&fr.convert_to_bytes());

        //---------------------

        let mut file = File::open(fr_file_name).unwrap();

        let mut buf = [0; 32];
        // let mut bytes = vec![];
        // file.read_to_end(&mut bytes).unwrap();

        file.read(&mut buf);
        let fr = Fr::convert_from_bytes(&buf);

        println!("{:?}", fr.0);
        println!("{:?}", fr);
    }

    #[test]
    fn test_io_fq() {
        let fq = Fq::one();

        println!("{:?}", fq.0);
        println!("{:?}", fq);
        println!("{:?}", fq.convert_to_bytes());
        write_field_into_file(fq_file_name, fq);

        let fq: Fq = read_field_from_file(fq_file_name);
        println!("{:?}", fq.0);
        println!("{:?}", fq);
        println!("{:?}", fq.convert_to_bytes());
    }

    #[test]
    fn test_io_fqs() {
        let fq = Fq::one();
        let fq2 = Fq::zero();

        println!("{:?}", fq.0);
        println!("{:?}", fq2.0);
        write_fields_into_file(fq_file_name, &[fq, fq2]);

        let fqs: Vec<Fq> = read_fields_from_file(fq_file_name, 2);
        println!("{:?}", fqs.get(0).unwrap().0);
        println!("{:?}", fqs.get(1).unwrap().0);
    }
}
