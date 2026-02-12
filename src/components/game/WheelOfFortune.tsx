"use client";

import React, { useState, useCallback } from "react";
import { playGame, canPlayGame } from "@/app/mall-actions";
import { Session } from "next-auth";
import { toast } from "sonner";

interface WheelOfFortuneProps {
  session: Session | null;
  initialCanPlay: boolean;
  initialReason: string;
}

const WHEEL_SEGMENTS = [
  { label: "10 CHF", color: "#FF6B6B" },
  { label: "Perdu", color: "#4ECDC4" },
  { label: "20 CHF", color: "#45B7D1" },
  { label: "Perdu", color: "#96CEB4" },
  { label: "30 CHF", color: "#FFEAA7" },
  { label: "Perdu", color: "#DDA0DD" },
  { label: "40 CHF", color: "#98D8C8" },
  { label: "Perdu", color: "#F7DC6F" },
];

export default function WheelOfFortune({
  session,
  initialCanPlay,
  initialReason,
}: WheelOfFortuneProps) {
  const [isSpinning, setIsSpinning] = useState(false);
  const [rotation, setRotation] = useState(0);
  const [canPlay, setCanPlay] = useState(initialCanPlay);
  const [reason, setReason] = useState(initialReason);
  const [result, setResult] = useState<{
    won: boolean;
    voucher?: { code: string; value: number; shopName: string };
  } | null>(null);

  const spin = useCallback(async () => {
    if (!session?.user) {
      toast.error("Vous devez vous connecter pour jouer");
      return;
    }

    if (!canPlay || isSpinning) return;

    setIsSpinning(true);
    setResult(null);

    // Start spinning animation
    const spins = 5 + Math.random() * 3; // 5-8 full rotations
    const extraDegrees = Math.random() * 360;
    const totalRotation = rotation + spins * 360 + extraDegrees;
    setRotation(totalRotation);

    // Call the server action
    const gameResult = await playGame();

    // Wait for animation to finish
    setTimeout(() => {
      setIsSpinning(false);

      if (gameResult.success) {
        if (gameResult.won && gameResult.voucher) {
          setResult({
            won: true,
            voucher: gameResult.voucher,
          });
          toast.success(
            `Félicitations! Vous avez gagné ${gameResult.voucher.value} CHF chez ${gameResult.voucher.shopName}!`
          );
        } else {
          setResult({ won: false });
          if (gameResult.canPlayAgain) {
            toast.info("Pas de chance! Vous pouvez réessayer une fois.");
          } else {
            toast.info("Pas de chance! Revenez demain pour une nouvelle tentative.");
            setCanPlay(false);
            setReason("max_attempts");
          }
        }

        // Update play status
        if (!gameResult.canPlayAgain) {
          setCanPlay(false);
        }
      } else {
        toast.error("Une erreur est survenue. Veuillez réessayer.");
      }
    }, 4000); // Match animation duration
  }, [session, canPlay, isSpinning, rotation]);

  if (!session?.user) {
    return (
      <div className="bg-gradient-to-br from-purple-600 to-blue-500 rounded-2xl p-8 text-center text-white">
        <h2 className="text-2xl font-bold mb-4">Roue de la Fortune</h2>
        <p className="mb-6">Connectez-vous pour tenter de gagner des bons d'achat!</p>
        <a
          href="/login"
          className="inline-block bg-white text-purple-600 font-semibold px-6 py-3 rounded-lg hover:bg-gray-100 transition"
        >
          Se connecter
        </a>
      </div>
    );
  }

  return (
    <div className="bg-gradient-to-br from-purple-600 to-blue-500 rounded-2xl p-8 text-white">
      <h2 className="text-2xl font-bold mb-2 text-center">Roue de la Fortune</h2>
      <p className="text-center text-purple-100 mb-6">
        Tentez votre chance et gagnez des bons d'achat!
      </p>

      <div className="relative w-64 h-64 mx-auto mb-6">
        {/* Wheel */}
        <div
          className="w-full h-full rounded-full border-4 border-white shadow-2xl overflow-hidden transition-transform duration-[4000ms] ease-out"
          style={{ transform: `rotate(${rotation}deg)` }}
        >
          <svg viewBox="0 0 100 100" className="w-full h-full">
            {WHEEL_SEGMENTS.map((segment, i) => {
              const angle = 360 / WHEEL_SEGMENTS.length;
              const startAngle = i * angle - 90;
              const endAngle = startAngle + angle;
              const startRad = (startAngle * Math.PI) / 180;
              const endRad = (endAngle * Math.PI) / 180;
              const x1 = 50 + 50 * Math.cos(startRad);
              const y1 = 50 + 50 * Math.sin(startRad);
              const x2 = 50 + 50 * Math.cos(endRad);
              const y2 = 50 + 50 * Math.sin(endRad);
              const largeArc = angle > 180 ? 1 : 0;

              const textAngle = startAngle + angle / 2;
              const textRad = (textAngle * Math.PI) / 180;
              const textX = 50 + 30 * Math.cos(textRad);
              const textY = 50 + 30 * Math.sin(textRad);

              return (
                <g key={i}>
                  <path
                    d={`M 50 50 L ${x1} ${y1} A 50 50 0 ${largeArc} 1 ${x2} ${y2} Z`}
                    fill={segment.color}
                  />
                  <text
                    x={textX}
                    y={textY}
                    textAnchor="middle"
                    dominantBaseline="middle"
                    fontSize="6"
                    fontWeight="bold"
                    fill="#333"
                    transform={`rotate(${textAngle + 90}, ${textX}, ${textY})`}
                  >
                    {segment.label}
                  </text>
                </g>
              );
            })}
          </svg>
        </div>

        {/* Pointer */}
        <div className="absolute top-0 left-1/2 -translate-x-1/2 -translate-y-2">
          <div className="w-0 h-0 border-l-[12px] border-l-transparent border-r-[12px] border-r-transparent border-t-[24px] border-t-yellow-400 drop-shadow-lg" />
        </div>
      </div>

      {/* Result Display */}
      {result && (
        <div
          className={`text-center p-4 rounded-lg mb-4 ${
            result.won ? "bg-green-500" : "bg-red-500"
          }`}
        >
          {result.won && result.voucher ? (
            <>
              <p className="font-bold text-lg">Félicitations!</p>
              <p>
                Vous avez gagné {result.voucher.value} CHF chez {result.voucher.shopName}
              </p>
              <p className="mt-2 font-mono bg-white/20 px-2 py-1 rounded inline-block">
                Code: {result.voucher.code}
              </p>
            </>
          ) : (
            <p className="font-bold">Pas de chance cette fois!</p>
          )}
        </div>
      )}

      {/* Spin Button */}
      <div className="text-center">
        {canPlay ? (
          <button
            onClick={spin}
            disabled={isSpinning}
            className={`px-8 py-4 rounded-full font-bold text-lg transition transform ${
              isSpinning
                ? "bg-gray-400 cursor-not-allowed"
                : "bg-yellow-400 text-purple-900 hover:bg-yellow-300 hover:scale-105"
            }`}
          >
            {isSpinning ? "La roue tourne..." : "TOURNER LA ROUE!"}
          </button>
        ) : (
          <div className="text-purple-100">
            {reason === "already_won" && (
              <p>Vous avez déjà gagné aujourd'hui! Revenez demain.</p>
            )}
            {reason === "max_attempts" && (
              <p>Vous avez utilisé vos 2 tentatives. Revenez demain!</p>
            )}
          </div>
        )}
      </div>

      <p className="text-center text-purple-200 text-sm mt-4">
        1 partie par jour • 2ème chance si vous perdez • 10 lots max par jour
      </p>
    </div>
  );
}
