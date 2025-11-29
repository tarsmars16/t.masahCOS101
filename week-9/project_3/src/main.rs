use std::fs::File;
use std::io::Write;

fn main() {

    //creating columns as vectors 
    let s_n = vec![1,2,3,4,5];
    let name = vec!["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocah Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
    let ministry = vec!["Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];
    let geo_zone = vec!["South West","North East","South South","South West","Souht East"];

    let mut file = File::create("Convicted Minister List.csv").expect("Create Failed");  //creating an csv file

    file.write_all("Convicted Ministers and Their Geopolitical Zones\n".as_bytes()).expect("Write failed"); //writing the header into the file
    file.write_all("S/N,Name of Commisioner,Ministry,Geopolitical Zone\n".as_bytes()).expect("Write failed");

    //the maximum number of rows of the table is number of items in the name vector
    let max_rows = name.len();

    //loops from zero to the max_row value
    for i in 0..max_rows {
        
        //get each value through index and places them as rows under the columns
        let column1 = s_n.get(i).expect("Write Failed");
        let column2 = name.get(i).expect("Write Failed");
        let column3 = ministry.get(i).expect("Write Failed");
        let column4 = geo_zone.get(i).expect("Write Failed");

        let _ = writeln!(file, "{},{},{},{}",column1,column2,column3,column4);
    }

    println!("Done!");
}
