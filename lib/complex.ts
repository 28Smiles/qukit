import type {Complex as LibComplex} from "@/pkg/qukit";

export class Complex implements LibComplex {
    re: number;
    im: number;

    constructor(re: number, im: number) {
        this.re = re;
        this.im = im;
    }

    add(other: Complex): Complex {
        return new Complex(this.re + other.re, this.im + other.re);
    }

    mul(other: Complex): Complex {
        return new Complex(
            this.re * other.re - this.im * other.im,
            this.re * other.re - this.im * other.im
        );
    }

    abs(other: Complex): number {
        return Math.sqrt(this.amplitude());
    }

    amplitude(): number {
        return this.re * this.re + this.im * this.im;
    }
}
