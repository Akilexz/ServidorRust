import { Component, OnInit } from '@angular/core';
import { environment } from 'src/environments/environment';
import { HttpClient } from '@angular/common/http';
import { Homework } from 'src/modulos/homework';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit{
  title = 'fronendTareas';

  responseHomeworks: any[];
  data: any;
  homeworks: Homework;
  statusNumber: any;
  constructor(private http: HttpClient){}

  ngOnInit():void {
    this.homeworks = {
      title: '',
      published: true
    };
    
    this.getAllHomeworks();
  }

  getAllHomeworks = () => {
    this.http.get<any>(environment.Api_url + 'homeworks').subscribe(data => {
      this.responseHomeworks = data.data;
      console.log(this.responseHomeworks);
      let datosBool =[];
      for(let i=0; i<this.responseHomeworks.length; i++){
        datosBool.push(this.responseHomeworks[i].published);
      }
      console.log(datosBool);
      let removedFalse = [];
      for(let i=0; i<datosBool.length; i++){
        if(datosBool[i] !== false){
          removedFalse.push(datosBool[i]);
        }
      }
      this.statusNumber = removedFalse.length;

    })
  }
  postSaveHomework = () => {
    this.data = {
        title: this.homeworks.title,
        published: true
    }
    this.http.post(environment.Api_url + 'homework', this.data).subscribe(response => {
      console.log(response);
      window.location.reload();
    });
  }

  deleteHomework = (id: number) => {
    console.log(id);
    if (id !== undefined){
      this.http.delete(environment.Api_url + `homework/${id}`).subscribe(response =>{
        console.log(response);
        window.location.reload();
      })
    }else{
      console.error("Error de id comuniquese con el administrador");
    }
  }

  editStatus = (id:number, title:any) => {
    console.log(id);
    if(id !== undefined) {
      this.data = {
        title: title,
        published: false
    }
      this.http.put(environment.Api_url + `homework/${id}`, this.data).subscribe(response=>{
        console.log(response);
        window.location.reload();
      })
    }else{
      console.error("Error de id comuniquese con el administrador");
    }
  }

  editHomework = (id:number,title:any, published:boolean) => {
    if(id !== undefined) {
      this.data = {
        title: title,
        published:published
      }
      this.http.put(environment.Api_url + `homework/${id}`,this.data).subscribe(response=>{
        console.log(response);
        window.location.reload();
      })
    }
  }
}
