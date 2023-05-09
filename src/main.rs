use std::io;

fn convert_to_int(data_input: & String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();    // Convert String to Int
    x
}

fn main() {
    let mut alunos = String::new();
    println!("Digite quantos alunos ha na turma: ");
    io::stdin().read_line(&mut alunos).expect("Erro ao ler alunos"); // how many students there are in the class?
    let mut alunos1 = convert_to_int(&mut alunos);
    let mut repro = 0;

    while alunos1 != 0 {
        let mut soma = 0;
        let mut nota1_s = String :: new();                                  
        println!("Digite a nota 1 do aluno {}: ", alunos1);
        io::stdin().read_line(&mut nota1_s).expect("Erro ao ler nota1");
        let mut nota1 = convert_to_int(&mut nota1_s);

                                                                                                // Write here student's grades
        let mut nota2_s = String :: new();
        println!("Digite a nota 2 do aluno {}: ", alunos1);
        io::stdin().read_line(&mut nota2_s).expect("Erro ao ler nota2");
        let mut nota2 = convert_to_int(&mut nota2_s);

        soma = (nota1 + nota2)/2;

        if soma < 6 {
            repro = repro + 1;                    // Student average calculation
        }
        alunos1 = alunos1 - 1;

        
    }
    println!("Há na turma {} alunos reprovados.", repro);  // Failed students
}



