import firebase from "firebase/app";
import "firebase/auth";
import "firebase/firestore";

const firebaseConfig = {
    apiKey: "AIzaSyBSiByly5HPoGuxNFdE-FVP3aUD9Y0ak7w",
    authDomain: "tech-news-77085.firebaseapp.com",
    databaseURL: "https://tech-news-77085.firebaseio.com",
    projectId: "tech-news-77085",
    storageBucket: "tech-news-77085.appspot.com",
    messagingSenderId: "237188242486",
    appId: "1:237188242486:web:3bba1d791850bac3c34c97"
};

export const firebaseApp = firebase.initializeApp(firebaseConfig);
const baseDB = firebaseApp.firestore();
export const db = baseDB;
