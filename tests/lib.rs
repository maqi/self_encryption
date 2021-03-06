/*  Copyright 2014 MaidSafe.net limited

    This MaidSafe Software is licensed to you under (1) the MaidSafe.net Commercial License,
    version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
    licence you accepted on initial access to the Software (the "Licences").

    By contributing code to the MaidSafe Software, or to this project generally, you agree to be
    bound by the terms of the MaidSafe Contributor Agreement, version 1.0, found in the root
    directory of this project at LICENSE, COPYING and CONTRIBUTOR respectively and also
    available at: http://www.maidsafe.net/licenses

    Unless required by applicable law or agreed to in writing, the MaidSafe Software distributed
    under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS
    OF ANY KIND, either express or implied.

    See the Licences for the specific language governing permissions and limitations relating to
    use of the MaidSafe
    Software.                                                                 */

extern crate self_encryption;
extern crate rand;
pub use self_encryption::*;
/// DataMap integratoin tests
#[test]
fn data_map_empty(){
  let dm = self_encryption::datamap::DataMap::Content(vec![110,111]);
  assert_eq!(dm.len(), 2);
  }

#[test]
fn data_map_content_only(){
  let dm = self_encryption::datamap::DataMap::Content(vec![110,111]);
  assert!(dm.len() == 2);
  assert!(dm.has_chunks() == false);
  }

fn random_string(length: u64) -> String {
        (0..length).map(|_| (0x20u8 + (rand::random::<f32>() * 96.0) as u8) as char).collect()
  }
/// Self Enryptor integration tests
/*#[test]
fn check_write() {
  let mut se = SelfEncryptor::new(&mut my_storage as &mut Storage, datamap::DataMap::None);
  se.write(&random_string(3), 5u64);
  assert_eq!(se.len(), 8u64);
}*/


pub struct MyStorage {
    name: Vec<u8>
}

impl Storage for MyStorage {
   //let mut name: Vec<u8> = vec![0x11];
   fn get(&self, name: Vec<u8>) -> Vec<u8> {
       name
       }
   fn put(&self, name: Vec<u8>, data: Vec<u8>){}
   }

#[test]
fn check_write() {
  let name = vec![0x11];
  let mut my_storage = MyStorage{name: vec![0x11]};
  let mut se = SelfEncryptor::new(&mut my_storage as &mut Storage, datamap::DataMap::None);
  se.write(&random_string(3), 5u64);
  assert_eq!(se.len(), 8u64);
  assert_eq!(se.get_storage().get(name),vec![0x11]);
}
