use clap::Parser;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::{thread, time};
use colored::*;
use std::io::{self};
use crossterm::{
    ExecutableCommand,
    terminal, cursor
};
use std::collections::{HashMap, BTreeMap};

/// Tool for monitoring changes in memory ranges
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    ///PID of the process used to attach to memory
    #[clap(short, long)]
    pid: u64,

    ///Where to start monitoring
    #[clap(default_value_t=0,short, long)]
    offset: u64,

    ///Size of the memory to monitor
    #[clap(default_value_t=256,short, long)]
    size: u64,

     ///Interval to scan in milliseconds
     #[clap(default_value_t=100,short, long)]
     interval: u64,
}

fn compare_vectors(buf1: Vec<u8>, buf2: Vec<u8>, size: u64) -> HashMap<usize, String>{
    let byte_len = 8 as usize;
    let screen_len = 4 as usize;

    let mut found: HashMap<usize, String> = HashMap::new();
    let mut found_preview: BTreeMap<usize, u8> = BTreeMap::new();

    for byte_pos in 0..size as usize {
        if buf1[byte_pos] == buf2[byte_pos] {
            print!("{:02x}", buf2[byte_pos]);
        } else {
            let b = format!("{:02x}", buf2[byte_pos]);
            print!("{}", b.red().bold());
            found_preview.insert(byte_pos, buf2[byte_pos].clone());
        }
        if (byte_pos+1) % byte_len == 0 {
            print!("\t");
        }
        if (byte_pos+1) % (byte_len * screen_len) == 0 {
            println!("");
        }
    }
    
    if found_preview.len() > 0 {
        let mut first_change = found_preview.first_key_value().unwrap().0;

        for (k,v) in &found_preview {
            if k-first_change > 1 {
                if *k > 0 && !found_preview.contains_key(&(*k-1)) {
                    first_change = k; 
                }
            } 
            if found.contains_key(first_change) {
                let s = found.get_mut(first_change).unwrap();
                *s = format!("{}{}", s, v.clone());
            } else {
                found.insert(*first_change, format!("{:02x}", v.clone()));
            }
        }
    }
    found
}

fn watch_memory_offset(pid: u64, offset: u64, size: u64, interval: u64) {
    let sleep_time: time::Duration = time::Duration::from_millis(interval);

    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    stdout.execute(cursor::MoveTo(0,0)).unwrap();
    stdout.flush().unwrap();

    print!("Watch PID {} at offset {} - press CTRL-C for end\n\n", pid, offset);
    let fname = format!("/proc/{}/mem", pid); //memory file 
    let mut f = File::open(fname).unwrap();

    let mut buffer1 = vec![0_u8; size as usize];
    let mut buffer2 = vec![0_u8; size as usize];
    let mut changed_buffers: BTreeMap<usize, String> = BTreeMap::new();

    loop {
        stdout.execute(cursor::MoveTo(0,3)).unwrap();
        f.seek(SeekFrom::Start(offset)).unwrap();
        f.read(&mut buffer1).unwrap();
        thread::sleep(sleep_time);

        f.seek(SeekFrom::Start(offset)).unwrap();
        f.read(&mut buffer2).unwrap();
        let found = compare_vectors(buffer1.clone(), buffer2.clone(), size);
        for (k, v) in found {
            if changed_buffers.contains_key(&k) {
                let s = changed_buffers.get_mut(&k).unwrap();
                *s = format!("{} => {}", s, v.clone());
            }
            else {
                changed_buffers.insert(k, v);
            }
        }

        println!("\nChanged offsets:");
        for (k, v) in &changed_buffers {
            println!("@{: <8}:    {}", k, v);
        }
    }
}

fn main() {
    let args = Args::parse(); 
    watch_memory_offset(args.pid, args.offset, args.size, args.interval);
}
