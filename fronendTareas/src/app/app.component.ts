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
    })
  }
  postSaveHomework = () => {
    this.data = {
        title: this.homeworks.title,
        published: true
    }
    this.http.post(environment.Api_url + 'homework', this.data).subscribe(response => {
      console.log(response);
    });
  }
}
