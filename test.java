class WhatHappen {
    public static void main(String[] args) {
        int handle;
        MyClient mc;
        mc = new MyClient();
        while(!false){
            handle = mc.start(10, 10);
        }
    }
}

class Client {
    int in;
    int out;
    int[] messagelist;
    int index;
    public boolean init(){
        index = 0;
        messagelist = new int[10];
        in = 0;
        out = 0;
        return true;
    }
    public int run(int host, int port){
        int handle;
        handle = this.Juggling();
        return handle;
    } 

    public int getMsg(){
        int tmp; 
        tmp = messagelist.length;
        if(this.isVoid()){
            tmp = tmp - 1;
        }
        else{
            tmp = tmp * 2;
        }
        if(index < 10){
            messagelist[index] = tmp;
            index = index + 1;
        }
        else{
            index = 0;
        }
        return tmp;
    }

    public boolean isVoid(){
        boolean flag;
        if(0 < messagelist.length){
            flag = false;
        }
        else{
            flag = true;
        }
        return flag;
    }

    public int Juggling(){
        boolean t;
        int tmp1;
        int tmp2;
        int tmp3;
        tmp1 = 2;
        tmp2 = 3;
        tmp3 = 4;
        while((tmp2 < tmp3)&&(tmp1<tmp2)){
            tmp1 = tmp3 - tmp2;
            tmp2 = tmp2 - tmp1;
            tmp3 = tmp2 * tmp1;
            t = this.HolyLight();
        }
        return (tmp1*tmp2+tmp3)*messagelist.length;
    }


    public boolean HolyLight(){
        in = in + 1;
        out = out - 1;
        System.out.println(false);
        return false;
    }
}

class MyClient extends Client{

    public int start(int host, int port){
        int handle;
        handle = this.run(127, 8081);
        return handle;
    }
}
