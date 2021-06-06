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

  // Variable para guardar los datos de la peticion get
  responseHomeworks: any[];
  // lo vamos a utilizar para el cuerpo de la peticion tanto como post y put
  data: any;
  // Modelo de la tabla tarea
  homeworks: Homework;
  // lo vamos a utilizar para recuperar cuantas tareas estan activas
  statusNumber: any;
  constructor(private http: HttpClient){}

  ngOnInit():void {
    this.homeworks = {
      title: '',
      published: true
    };
    
    this.getAllHomeworks();
  }

  // funcion para traer todas las tareas
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

  // funcion para guardar una nueva tarea
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

  // Funcion para eliminar una tarea
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

  // Funcion para editar el estado del campo seleccionado
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

  // Funcion para editar el texto del campo seleccionado
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
