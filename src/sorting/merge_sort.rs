use std::fmt::Debug;
pub fn merge_sort<T>(collection:&[T])->Vec<T> where T:PartialOrd+Clone+Debug {
     if collection.len()>1{
         let (l,r)=collection.split_at(collection.len()/2);
         let sorted_l=merge_sort(l);
         let sorted_r=merge_sort(r);
         let mut result:Vec<T>=collection.into();
         let (mut i,mut j)=(0,0);
         let mut k=0;
         while i<sorted_l.len() &&j<sorted_r.len(){
             if sorted_l[i]<=sorted_r[j]{
                 result[k]=sorted_l[i].clone();
                 i+=1;
             }else{
                 result[k]=sorted_r[j].clone();
                 j+=1;

             }
             k+=1;
         }

         while i<sorted_l.len(){
             result[k]=sorted_l[i].clone();
             k+=1;
             i+=1;
         }
         while j<sorted_r.len(){
             result[k]=sorted_r[j].clone();
             k+=1;
             j+=1;
         }
         result
     }   else{
         collection.to_vec()
     }
}