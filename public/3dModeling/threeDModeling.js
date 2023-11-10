$(document).ready(function() {
    // Create 3D models using Three.js
    var scene1 = new THREE.Scene();
    var camera1 = new THREE.PerspectiveCamera(75, 1, 0.1, 1000);
    var renderer1 = new THREE.WebGLRenderer({ alpha: true });
    renderer1.setSize($("#model1").width(), $("#model1").height());
    $("#model1").append(renderer1.domElement);
    var geometry1 = new THREE.BoxGeometry();
    var material1 = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
    var cube1 = new THREE.Mesh(geometry1, material1);
    scene1.add(cube1);
    camera1.position.z = 5;
    function animate1() {
        requestAnimationFrame(animate1);
        cube1.rotation.x += 0.01;
        cube1.rotation.y += 0.01;
        renderer1.render(scene1, camera1);
    }
    animate1();
 
    var scene2 = new THREE.Scene();
    var camera2 = new THREE.PerspectiveCamera(75, 1, 0.1, 1000);
    var renderer2 = new THREE.WebGLRenderer({ alpha: true });
    renderer2.setSize($("#model2").width(), $("#model2").height());
    $("#model2").append(renderer2.domElement);
    var geometry2 = new THREE.SphereGeometry();
    var material2 = new THREE.MeshBasicMaterial({ color: 0xff0000 });
    var sphere2 = new THREE.Mesh(geometry2, material2);
    scene2.add(sphere2);
    camera2.position.z = 5;
    function animate2() {
        requestAnimationFrame(animate2);
        sphere2.rotation.x += 0.01;
        sphere2.rotation.y += 0.01;
        renderer2.render(scene2, camera2);
    }
    animate2();
 
    var scene3 = new THREE.Scene();
    var camera3 = new THREE.PerspectiveCamera(75, 1, 0.1, 1000);
    var renderer3 = new THREE.WebGLRenderer({ alpha: true });
    renderer3.setSize($("#model3").width(), $("#model3").height());
    $("#model3").append(renderer3.domElement);
    var geometry3 = new THREE.ConeGeometry();
    var material3 = new THREE.MeshBasicMaterial({ color: 0x0000ff });
    var cone3 = new THREE.Mesh(geometry3, material3);
    scene3.add(cone3);
    camera3.position.z = 5;
    function animate3() {
        requestAnimationFrame(animate3);
        cone3.rotation.x += 0.01;
        cone3.rotation.y += 0.01;
        renderer3.render(scene3, camera3);
    }
    animate3();
});